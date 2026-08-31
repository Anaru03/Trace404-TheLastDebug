mod bmp;
mod camera;
mod framebuffer;
mod ray;
mod sphere;
mod vector;

use bmp::save_bmp;
use camera::Camera;
use framebuffer::{Framebuffer, rgb};
use sphere::Sphere;
use vector::Vec3;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 60.0_f32.to_radians());

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0);

    let light_position = Vec3::new(-3.0, 3.0, 0.0);

    framebuffer.clear(rgb(20, 20, 30));

    let mut hits = 0;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, HEIGHT);

            let color = if let Some(t) = sphere.intersect(&ray) {
                hits += 1;

                let hit_point = ray.at(t);

                let normal = sphere.normal_at(hit_point);

                let light_direction = (light_position - hit_point).normalize();

                let intensity = normal.dot(&light_direction).max(0.0);

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
    println!("Rayos que golpearon la esfera: {}", hits);
    println!("Render guardado en render.bmp");
}
