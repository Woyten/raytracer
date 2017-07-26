use object::Object;
use prelude::*;
use ray;
use ray::Ray;

pub struct Plane<F> {
    normal: Vector3,
    normal_offset: f64,
    reflectivity: f64,
    color_fn: F,
}

impl<F: Fn(&Point3) -> Color> Plane<F> {
    pub fn from_triangle(point_a: Point3, point_b: Point3, point_c: Point3, reflectivity: f64, color_fn: F) -> Plane<F> {
        let ab = point_b - point_a;
        let bc = point_c - point_b;
        let normal = ab.cross(&bc);
        Plane {
            normal,
            normal_offset: normal.dot(&point_a.coords),
            reflectivity,
            color_fn,
        }
    }
}

impl<F: Fn(&Point3) -> Color> Object for Plane<F> {
    fn get_alpha(&self, ray: &Ray) -> Option<f64> {
        let alpha = (self.normal_offset - self.normal.dot(&ray.start.coords)) / self.normal.dot(&ray.direction);

        if alpha < 1e-9 { None } else { Some(alpha) }
    }

    fn get_color(&self, ray: &Ray, alpha: f64, scene: &[&Object], num_recursions: usize) -> Color {
        let reflection_point = ray.start + alpha * ray.direction;
        let color = (self.color_fn)(&reflection_point);

        if num_recursions == 0 {
            return color;
        }

        let reflected_ray = Ray {
            start: reflection_point,
            direction: ray::reflect(&ray.direction, &self.normal),
        };

        let reflected_color = reflected_ray.trace(scene, num_recursions - 1);
        color + self.reflectivity * reflected_color
    }
}