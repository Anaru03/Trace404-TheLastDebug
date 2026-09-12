use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Cylinder {
    pub center: Vec3,
    pub radius: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f32, min_y: f32, max_y: f32) -> Self {
        Self {
            center,
            radius,
            min_y,
            max_y,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let ox = ray.origin.x - self.center.x;
        let oz = ray.origin.z - self.center.z;

        let dx = ray.direction.x;
        let dz = ray.direction.z;

        let a = dx * dx + dz * dz;
        let b = 2.0 * (ox * dx + oz * dz);
        let c = ox * ox + oz * oz - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 || a.abs() < 0.000001 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut t1 = (-b - sqrt_d) / (2.0 * a);
        let mut t2 = (-b + sqrt_d) / (2.0 * a);

        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }

        for t in [t1, t2] {
            if t > 0.001 {
                let y = ray.origin.y + ray.direction.y * t;

                if y >= self.min_y && y <= self.max_y {
                    return Some(t);
                }
            }
        }

        None
    }

    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        Vec3::new(point.x - self.center.x, 0.0, point.z - self.center.z).normalize()
    }
}
