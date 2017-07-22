use prelude::*;
use ray::Ray;

pub struct Sphere {
    pub middle: Point3,
    pub radius: f64,
    pub color: Color,
}

impl Sphere {
    pub fn get_intersection_distance(&self, ray: &Ray) -> Option<f64> {
        let ms = ray.start - self.middle;
        let d_sqr = ray.direction.norm_squared();
        let p_half = ms.dot(&ray.direction) / d_sqr;
        let q = (ms.norm_squared() - self.radius * self.radius) / d_sqr;

        let discriminant = p_half * p_half - q;

        if discriminant < 0.0 {
            return None;
        }

        let nearest = -p_half - discriminant.sqrt();
        if nearest > 0.0 { Some(nearest) } else { None }
    }
}