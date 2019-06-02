use crate::material::Material;
use crate::object::Object;
use crate::prelude::*;
use crate::ray::Ray;

pub struct Sphere<M> {
    pub middle: Point3,
    pub radius: f64,
    pub material: M,
}

impl<M: Material + Sync> Object for Sphere<M> {
    fn get_alpha(&self, ray: &Ray, _: f64) -> Option<f64> {
        let ms = ray.start - self.middle;
        let d_sqr = ray.direction.norm_squared();
        let p_half = ms.dot(&ray.direction) / d_sqr;
        let q = (ms.norm_squared() - self.radius * self.radius) / d_sqr;

        let discriminant = p_half * p_half - q;

        if discriminant < 0.0 {
            return None;
        }
        let sqrt = discriminant.sqrt();

        let hit_from_outside = -p_half - sqrt;
        if hit_from_outside > 1e-9 {
            return Some(hit_from_outside);
        }

        let hit_from_inside = -p_half + sqrt;
        if hit_from_inside > 1e-9 {
            return Some(hit_from_inside);
        }

        None
    }

    fn get_color(
        &self,
        ray: &Ray,
        alpha: f64,
        scene: &[&dyn Object],
        num_recursions: usize,
    ) -> Color {
        let reflection_point = ray.start + alpha * ray.direction;
        let normal = reflection_point - self.middle;
        self.material.get_color(
            ray.direction,
            reflection_point,
            &normal,
            scene,
            num_recursions,
        )
    }
}
