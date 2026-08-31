mod bmp;
mod camera;
mod cube;
mod framebuffer;
mod light;
mod ray;
mod sphere;
mod vector;

use bmp::save_bmp;
use camera::Camera;
use cube::Cube;
use framebuffer::{Framebuffer, rgb};
use light::Light;
use sphere::Sphere;
use vector::Vec3;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 60.0_f32.to_radians());

    let spheres = [
        Sphere::new(Vec3::new(-1.3, 0.0, -5.0), 1.0),
        Sphere::new(Vec3::new(1.0, 0.2, -4.0), 0.8),
        Sphere::new(Vec3::new(0.0, -0.8, -6.0), 1.2),
    ];

    let cube = Cube::new(Vec3::new(-0.8, -0.8, -3.5), Vec3::new(0.8, 0.8, -2.5));

    let light = Light::new(Vec3::new(-3.0, 3.0, 0.0), 1.0);

    framebuffer.clear(rgb(20, 20, 30));

    let mut hits = 0;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, HEIGHT);

            let mut closest_t = f32::INFINITY;
            let mut closest_sphere: Option<&Sphere> = None;
            let mut hit_cube = false;

            for sphere in &spheres {
                if let Some(t) = sphere.intersect(&ray) {
                    if t < closest_t {
                        closest_t = t;
                        closest_sphere = Some(sphere);
                        hit_cube = false;
                    }
                }
            }

            if let Some(t) = cube.intersect(&ray) {
                if t < closest_t {
                    closest_t = t;
                    closest_sphere = None;
                    hit_cube = true;
                }
            }

            let color = if hit_cube {
                hits += 1;

                let hit_point = ray.at(closest_t);
                let normal = cube.normal_at(hit_point);

                let intensity = light.illuminate(hit_point, normal);

                let ambient = 0.1;
                let brightness = (ambient + intensity * 0.9).min(1.0);

                let r = (180.0 * brightness) as u32;
                let g = (80.0 * brightness) as u32;
                let b = (50.0 * brightness) as u32;

                rgb(r, g, b)
            } else if let Some(sphere) = closest_sphere {
                hits += 1;

                let hit_point = ray.at(closest_t);
                let normal = sphere.normal_at(hit_point);

                let intensity = light.illuminate(hit_point, normal);

                let ambient = 0.1;
                let brightness = (ambient + intensity * 0.9).min(1.0);

                let r = (0.0 * brightness) as u32;
                let g = (200.0 * brightness) as u32;
                let b = (180.0 * brightness) as u32;

                rgb(r, g, b)
            } else {
                rgb(20, 20, 30)
            };

            framebuffer.set_pixel(x, y, color);
        }
    }

    save_bmp(&framebuffer, "render.bmp").expect("No se pudo guardar el render");

    println!("TRACE//404: The Last Debug");
    println!("Rayos generados: {}", WIDTH * HEIGHT);
    println!("Rayos que golpearon un objeto: {}", hits);
    println!("Render guardado en render.bmp");
}
