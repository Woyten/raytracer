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

        let alpha = -p_half - discriminant.sqrt();
        if alpha > 0.0 { Some(alpha) } else { None }
    }

    pub fn get_reflected_ray(&self, ray: &Ray, alpha: f64) -> Ray {
        let reflection_point = ray.start + alpha * ray.direction;
        let normal = reflection_point - self.middle;

        let reflected_direction = ray.direction - 2.0 * ray.direction.dot(&normal) * normal / normal.norm_squared();
        Ray {
            start: reflection_point,
            direction: reflected_direction,
        }
    }
}