use crate::ray::Ray;
use crate::vector::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub fov: f32,
}

impl Camera {
    pub fn new(position: Vec3, fov: f32) -> Self {
        Self { position, fov }
    }

    pub fn get_ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let aspect_ratio = width as f32 / height as f32;

        let pixel_x = (2.0 * (x as f32 + 0.5) / width as f32 - 1.0) * aspect_ratio;

        let pixel_y = 1.0 - 2.0 * (y as f32 + 0.5) / height as f32;

        let fov_scale = (self.fov / 2.0).tan();

        let direction = Vec3::new(pixel_x * fov_scale, pixel_y * fov_scale, -1.0).normalize();

        Ray::new(self.position, direction)
    }
}
