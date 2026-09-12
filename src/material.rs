use crate::framebuffer::rgb;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub color: u32,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflectivity: f32,
}

impl Material {
    pub fn new(
        color: u32,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
        reflectivity: f32,
    ) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            reflectivity,
        }
    }

    pub fn wood() -> Self {
        Self::new(rgb(105, 67, 42), 0.18, 0.82, 0.15, 16.0, 0.05)
    }

    pub fn dark_metal() -> Self {
        Self::new(rgb(38, 43, 47), 0.12, 0.65, 0.65, 64.0, 0.35)
    }

    pub fn plastic() -> Self {
        Self::new(rgb(48, 46, 41), 0.16, 0.75, 0.30, 32.0, 0.08)
    }

    pub fn screen() -> Self {
        Self::new(rgb(12, 105, 118), 0.28, 0.65, 0.45, 48.0, 0.15)
    }

    pub fn floor() -> Self {
        Self::new(rgb(58, 62, 68), 0.14, 0.80, 0.10, 8.0, 0.02)
    }

    pub fn server() -> Self {
        Self::new(rgb(45, 51, 55), 0.12, 0.70, 0.40, 48.0, 0.20)
    }

    pub fn blue_led() -> Self {
        Self::new(rgb(20, 120, 180), 0.80, 0.20, 0.20, 8.0, 0.0)
    }

    pub fn green_led() -> Self {
        Self::new(rgb(20, 170, 90), 0.80, 0.20, 0.20, 8.0, 0.0)
    }
}
