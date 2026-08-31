use crate::vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vec3, intensity: f32) -> Self {
        Self {
            position,
            intensity,
        }
    }

    pub fn illuminate(&self, point: Vec3, normal: Vec3) -> f32 {
        let light_direction = (self.position - point).normalize();

        normal.dot(&light_direction).max(0.0) * self.intensity
    }
}
