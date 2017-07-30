use material::Material;
use object::Object;
use prelude::*;
use ray::Ray;

pub struct Sphere<M> {
    pub middle: Point3,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Object for Sphere<M> {
    fn get_alpha(&self, ray: &Ray) -> Option<f64> {
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

    fn get_color(&self, ray: &Ray, alpha: f64, scene: &[&Object], num_recursions: usize) -> Color {
        let reflection_point = ray.start + alpha * ray.direction;
        let normal = reflection_point - self.middle;
        self.material.get_color(ray.direction, reflection_point, &normal, scene, num_recursions )
    }
}