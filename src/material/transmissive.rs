use material::Material;
use object::Object;
use prelude::*;
use ray;
use ray::Ray;

pub struct Transmissive {
    pub color: Color,
    pub refraction: f64,
    pub reflectivity: f64,
}

impl Material for Transmissive {
    fn get_color(&self, direction: Vector3, reflection_point: Point3, normal: &Vector3, scene: &[&Object], num_recursions: usize) -> Color {
        if num_recursions == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let normalized_normal = normal.normalize();
        let normal_projection = normalized_normal.dot(&direction);
        let refraction = if normal_projection > 0.0 { self.refraction.recip() } else { self.refraction };
        let tangential_space = direction - normal_projection * normalized_normal;
        let discriminant = refraction * refraction * normal_projection * normal_projection + (refraction * refraction - 1.0) * tangential_space.norm_squared();

        let reflected_ray = Ray {
            start: reflection_point,
            direction: ray::reflect(&direction, &normal),
        };
        let reflected_color = reflected_ray.trace(scene, num_recursions - 1);

        let is_total_reflection = discriminant < 0.0;
        if is_total_reflection {
            reflected_color
        } else {
            let refracted_ray = Ray {
                start: reflection_point,
                direction: tangential_space + normal_projection.signum() * discriminant.sqrt() * normalized_normal,
            };
            let transmitted_color = self.color.component_mul(
                &refracted_ray.trace(scene, num_recursions - 1),
            );

            transmitted_color + self.reflectivity * reflected_color
        }
    }
}