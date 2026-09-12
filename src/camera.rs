use crate::ray::Ray;
use crate::vector::Vec3;
use std::f32::consts::PI;

const PITCH_LIMIT: f32 = PI / 2.0 - 0.1;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub fov: f32,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3, fov: f32) -> Self {
        Self {
            eye,
            center,
            up,
            fov,
        }
    }

    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        let rotated = right * vector.x + up * vector.y - forward * vector.z;

        rotated.normalize()
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.length();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);

        let radius_xz =
            (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();

        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);

        let new_pitch = (current_pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        self.eye = self.center
            + Vec3::new(
                radius * new_yaw.cos() * new_pitch.cos(),
                -radius * new_pitch.sin(),
                radius * new_yaw.sin() * new_pitch.cos(),
            );
    }

    pub fn get_ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let aspect_ratio = width as f32 / height as f32;
        let fov_scale = (self.fov / 2.0).tan();

        let pixel_x = (2.0 * (x as f32 + 0.5) / width as f32 - 1.0) * aspect_ratio * fov_scale;

        let pixel_y = (1.0 - 2.0 * (y as f32 + 0.5) / height as f32) * fov_scale;

        let camera_direction = Vec3::new(pixel_x, pixel_y, -1.0).normalize();

        let world_direction = self.basis_change(&camera_direction);

        Ray::new(self.eye, world_direction)
    }
}
