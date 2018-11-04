use crate::material::Material;
use crate::object::Object;
use crate::prelude::*;
use crate::ray;
use crate::ray::Ray;

pub struct Diffuse<F: Fn(&Point3) -> Color> {
    pub light_side: Vector3,
    pub color_fn: F,
    pub reflectivity: f64,
}

impl<F: Fn(&Point3) -> Color> Material for Diffuse<F> {
    fn get_color(&self, direction: Vector3, reflection_point: Point3, normal: &Vector3, scene: &[&Object], num_recursions: usize) -> Color {
        let dot_product = normal.normalize().dot(&self.light_side.normalize());
        let diffuse_color = (dot_product + 1.0) * 0.5 * (self.color_fn)(&reflection_point);

        if num_recursions == 0 {
            return diffuse_color;
        }

        let reflected_ray = Ray {
            start: reflection_point,
            direction: ray::reflect(&direction, &normal),
        };

        let reflected_color = self.reflectivity * reflected_ray.trace(scene, num_recursions - 1);
        diffuse_color + reflected_color
    }
}