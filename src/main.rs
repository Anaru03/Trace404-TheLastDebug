mod bmp;
mod camera;
mod cube;
mod framebuffer;
mod light;
mod ray;
mod scene;
mod sphere;
mod vector;

use camera::Camera;
use framebuffer::{Framebuffer, rgb};
use light::Light;
use minifb::{Key, Scale, Window, WindowOptions};
use scene::{SceneObject, build_scene};
use vector::Vec3;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

const ROTATION_SPEED: f32 = 0.05;

fn render(framebuffer: &mut Framebuffer, camera: &Camera, objects: &[SceneObject], light: &Light) {
    let background = rgb(15, 18, 24);

    framebuffer.clear(background);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, HEIGHT);

            let mut closest_t = f32::INFINITY;
            let mut pixel_color = background;

            for object in objects {
                if let Some(t) = object.cube.intersect(&ray) {
                    if t < closest_t {
                        closest_t = t;

                        let hit_point = ray.at(t);
                        let normal = object.cube.normal_at(hit_point);

                        let diffuse = light.illuminate(hit_point, normal);

                        let ambient = 0.22;
                        let brightness = (ambient + diffuse * 0.78).min(1.0);

                        let r = ((object.color >> 16) & 255) as f32;
                        let g = ((object.color >> 8) & 255) as f32;
                        let b = (object.color & 255) as f32;

                        pixel_color = rgb(
                            (r * brightness) as u32,
                            (g * brightness) as u32,
                            (b * brightness) as u32,
                        );
                    }
                }
            }

            framebuffer.set_pixel(x, y, pixel_color);
        }
    }
}

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let objects = build_scene();

    let light = Light::new(Vec3::new(-1.95, 1.10, -2.35), 1.35);

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
            }
        }

        render(&mut framebuffer, &camera, &objects, &light);

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}
