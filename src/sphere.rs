use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        if t1 > 0.0 {
            Some(t1)
        } else if t2 > 0.0 {
            Some(t2)
        } else {
            None
        }
    }

    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        (point - self.center).normalize()
    }
}
