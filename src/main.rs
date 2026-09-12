mod bmp;
mod camera;
mod cube;
mod framebuffer;
mod light;
mod ray;
mod sphere;
mod vector;

use camera::Camera;
use cube::Cube;
use framebuffer::{Framebuffer, rgb};
use light::Light;
use minifb::{Key, Window, WindowOptions};
use vector::Vec3;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

const ROTATION_SPEED: f32 = 0.05;

fn render(framebuffer: &mut Framebuffer, camera: &Camera, objects: &[(Cube, u32)], light: &Light) {
    let background = rgb(15, 18, 24);

    framebuffer.clear(background);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, HEIGHT);

            let mut closest_t = f32::INFINITY;
            let mut pixel_color = background;

            for (cube, base_color) in objects {
                if let Some(t) = cube.intersect(&ray) {
                    if t < closest_t {
                        closest_t = t;

                        let hit_point = ray.at(t);
                        let normal = cube.normal_at(hit_point);

                        let diffuse = light.illuminate(hit_point, normal);

                        let ambient = 0.15;
                        let brightness = (ambient + diffuse * 0.85).min(1.0);

                        let r = ((*base_color >> 16) & 255) as f32;
                        let g = ((*base_color >> 8) & 255) as f32;
                        let b = (*base_color & 255) as f32;

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

    let objects = vec![
        (
            Cube::new(Vec3::new(-4.5, -1.2, -7.0), Vec3::new(4.5, -1.0, 1.0)),
            rgb(55, 58, 65),
        ),
        (
            Cube::new(Vec3::new(-3.2, -0.8, -4.5), Vec3::new(3.2, -0.5, -2.0)),
            rgb(95, 65, 45),
        ),
        (
            Cube::new(Vec3::new(-1.6, -0.5, -4.0), Vec3::new(1.6, 1.4, -3.7)),
            rgb(35, 40, 45),
        ),
        (
            Cube::new(Vec3::new(-1.35, -0.25, -3.65), Vec3::new(1.35, 1.15, -3.55)),
            rgb(20, 80, 90),
        ),
        (
            Cube::new(Vec3::new(2.0, -0.5, -4.2), Vec3::new(3.2, 1.6, -3.0)),
            rgb(30, 35, 40),
        ),
        (
            Cube::new(Vec3::new(-4.0, -1.0, -6.5), Vec3::new(-3.2, 2.8, -5.5)),
            rgb(25, 30, 35),
        ),
        (
            Cube::new(Vec3::new(3.2, -1.0, -6.5), Vec3::new(4.0, 2.8, -5.5)),
            rgb(25, 30, 35),
        ),
    ];

    let light = Light::new(Vec3::new(-4.0, 6.0, 2.0), 1.0);

    let mut camera = Camera::new(
        Vec3::new(6.0, 4.0, 6.0),
        Vec3::new(0.0, 0.0, -3.5),
        Vec3::new(0.0, 1.0, 0.0),
        60.0_f32.to_radians(),
    );

    let mut window = Window::new(
        "TRACE//404: The Last Debug | Flechas: camara orbital | ESC: salir",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
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
