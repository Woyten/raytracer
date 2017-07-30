use material::Material;
use object::Object;
use prelude::*;
use ray::Ray;

pub struct Primitive<M> {
    normal: Vector3,
    normal_offset: f64,
    normal_a: Vector3,
    normal_a_offset: f64,
    normal_b: Vector3,
    normal_b_offset: f64,
    normal_c: Vector3,
    normal_c_offset: f64,
    material: M,
}

impl<M> Primitive<M> {
    pub fn new(point_a: Point3, point_b: Point3, point_c: Point3, material: M) -> Primitive<M> {
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
            material,
        }
    }
}

impl<M: Material> Object for Primitive<M> {
    fn get_alpha(&self, ray: &Ray) -> Option<f64> {
        let orthogonality = self.normal.dot(&ray.direction);
        if orthogonality == 0.0 {
            return None;
        }
        let alpha = (self.normal_offset - self.normal.dot(&ray.start.coords)) / orthogonality;
        if alpha <= 0.0 {
            return None;
        }

        let hit_point = (ray.start + alpha * ray.direction).coords;
        let is_inside = self.normal_a.dot(&hit_point) < 1e-9 + self.normal_a_offset && self.normal_b.dot(&hit_point) <= 1e-9 + self.normal_b_offset &&
            self.normal_c.dot(&hit_point) < 1e-9 + self.normal_c_offset;

        if is_inside { Some(alpha) } else { None }
    }

    fn get_color(&self, ray: &Ray, alpha: f64, scene: &[&Object], num_recursions: usize) -> Color {
        let reflection_point = ray.start + alpha * ray.direction;
        self.material.get_color(
            ray.direction,
            reflection_point,
            &self.normal,
            scene,
            num_recursions,
        )
    }
}