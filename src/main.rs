mod camera;
mod framebuffer;
mod ray;
mod vector;

use camera::Camera;
use framebuffer::{Framebuffer, rgb};
use vector::Vec3;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 60.0_f32.to_radians());

    framebuffer.clear(rgb(20, 20, 30));

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, HEIGHT);

            let direction = ray.direction;

            let r = ((direction.x + 1.0) * 0.5 * 255.0) as u32;
            let g = ((direction.y + 1.0) * 0.5 * 255.0) as u32;
            let b = 150;

            framebuffer.set_pixel(x, y, rgb(r, g, b));
        }
    }

    let center_ray = camera.get_ray(WIDTH / 2, HEIGHT / 2, WIDTH, HEIGHT);

    println!("TRACE//404: The Last Debug");
    println!("Rayos generados: {}", WIDTH * HEIGHT);
    println!("Rayo central: {:?}", center_ray);
}
