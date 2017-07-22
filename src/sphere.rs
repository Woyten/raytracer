use prelude::*;
use ray::Ray;

pub struct Sphere {
    pub middle: Point3,
    pub radius: f64,
    pub color: Color,
}

impl Sphere {
    pub fn has_intersection(&self, ray: &Ray) -> bool {
        let ms = ray.start - self.middle;
        let d_sqr = ray.direction.norm_squared();
        let p_half =  ms.dot(&ray.direction) / d_sqr;
        let q = (ms.norm_squared() - self.radius * self.radius) / d_sqr;

        p_half * p_half >  q
    }
}