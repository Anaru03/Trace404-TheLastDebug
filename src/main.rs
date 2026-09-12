mod bmp;
mod camera;
mod cube;
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

fn cast_shadow(point: Vec3, normal: Vec3, light: &Light, objects: &[SceneObject]) -> bool {
    let shadow_origin = point + normal * SHADOW_BIAS;

    let to_light = light.position - shadow_origin;
    let light_distance = to_light.length();
    let light_direction = to_light.normalize();

    let shadow_ray = Ray::new(shadow_origin, light_direction);

    for object in objects {
        if let Some(t) = object.cube.intersect(&shadow_ray) {
            if t > 0.0 && t < light_distance {
                return true;
            }
        }
    }

    false
}

fn render(framebuffer: &mut Framebuffer, camera: &Camera, objects: &[SceneObject], light: &Light) {
    let background = rgb(15, 18, 24);

    framebuffer.clear(background);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, HEIGHT);

            let mut closest_t = f32::INFINITY;
            let mut closest_object: Option<&SceneObject> = None;

            for object in objects {
                if let Some(t) = object.cube.intersect(&ray) {
                    if t < closest_t {
                        closest_t = t;
                        closest_object = Some(object);
                    }
                }
            }

            let pixel_color = if let Some(object) = closest_object {
                let hit_point = ray.at(closest_t);

                let normal = object.cube.normal_at(hit_point);

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

                let brightness =
                    (object.material.ambient + diffuse_light * object.material.diffuse).min(1.0);

                let color = object.material.color;

                let r = ((color >> 16) & 255) as f32;

                let g = ((color >> 8) & 255) as f32;

                let b = (color & 255) as f32;

                rgb(
                    (r * brightness + 255.0 * specular_light).min(255.0) as u32,
                    (g * brightness + 255.0 * specular_light).min(255.0) as u32,
                    (b * brightness + 255.0 * specular_light).min(255.0) as u32,
                )
            } else {
                background
            };

            framebuffer.set_pixel(x, y, pixel_color);
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
