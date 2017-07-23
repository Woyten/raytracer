use object::Object;
use prelude::*;
use ray::Ray;

pub struct Sphere {
    pub middle: Point3,
    pub radius: f64,
    pub color: Color,
    pub reflectivity: f64,
}

impl Object for Sphere {
    fn get_alpha(&self, ray: &Ray) -> Option<f64> {
        let ms = ray.start - self.middle;
        let d_sqr = ray.direction.norm_squared();
        let p_half = ms.dot(&ray.direction) / d_sqr;
        let q = (ms.norm_squared() - self.radius * self.radius) / d_sqr;

        let discriminant = p_half * p_half - q;

        if discriminant <= 0.0 {
            return None;
        }

        let alpha = -p_half - discriminant.sqrt();
        if alpha > 0.0 { Some(alpha) } else { None }
    }

    fn get_color(&self, ray: &Ray, alpha: f64, scene: &[&Object], num_recursions: usize) -> Color {
        if num_recursions == 0 {
            return self.color;
        }

        let reflection_point = ray.start + alpha * ray.direction;
        let normal = reflection_point - self.middle;

        let reflected_ray = Ray {
            start: reflection_point,
            direction: ray.direction - 2.0 * ray.direction.dot(&normal) * normal / normal.norm_squared(),
        };

        let reflected_color = reflected_ray.trace(scene, num_recursions - 1);
        self.color + self.reflectivity * reflected_color
    }
}