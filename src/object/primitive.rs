use object::Object;
use prelude::*;
use ray;
use ray::Ray;

pub struct Primitive {
    normal: Vector3,
    normal_offset: f64,
    normal_a: Vector3,
    normal_a_offset: f64,
    normal_b: Vector3,
    normal_b_offset: f64,
    normal_c: Vector3,
    normal_c_offset: f64,
    color: Vector3,
    reflectivity: f64,
}

impl Primitive {
    pub fn new(point_a: Point3, point_b: Point3, point_c: Point3, color: Color, reflectivity: f64) -> Primitive {
        let ab = point_b - point_a;
        let bc = point_c - point_b;
        let ca = point_a - point_c;
        let normal = ab.cross(&bc);
        let normal_a = bc.cross(&normal);
        let normal_b = ca.cross(&normal);
        let normal_c = ab.cross(&normal);
        Primitive {
            normal,
            normal_offset: normal.dot(&point_a.coords),
            normal_a,
            normal_a_offset: normal_a.dot(&point_b.coords),
            normal_b,
            normal_b_offset: normal_b.dot(&point_c.coords),
            normal_c,
            normal_c_offset: normal_c.dot(&point_a.coords),
            color,
            reflectivity,
        }
    }
}

impl Object for Primitive {
    fn get_alpha(&self, ray: &Ray) -> Option<f64> {
        let alpha = (self.normal_offset - self.normal.dot(&ray.start.coords)) / self.normal.dot(&ray.direction);

        if alpha < 1e-9 {
            return None;
        }

        let hit_point = (ray.start + alpha * ray.direction).coords;
        let is_inside = self.normal_a.dot(&hit_point) < 1e-9 + self.normal_a_offset && self.normal_b.dot(&hit_point) <= 1e-9 + self.normal_b_offset &&
            self.normal_c.dot(&hit_point) < 1e-9 + self.normal_c_offset;

        if is_inside { Some(alpha) } else { None }
    }

    fn get_color(&self, ray: &Ray, alpha: f64, scene: &[&Object], num_recursions: usize) -> Color {
        if num_recursions == 0 {
            return self.color;
        }

        let reflection_point = ray.start + alpha * ray.direction;

        let reflected_ray = Ray {
            start: reflection_point,
            direction: ray::reflect(&ray.direction, &self.normal),
        };

        let reflected_color = reflected_ray.trace(scene, num_recursions - 1);
        self.color + self.reflectivity * reflected_color
    }
}