use material::Material;
use object::Object;
use prelude::*;
use ray::Ray;

pub struct Plane<M> {
    normal: Vector3,
    normal_offset: f64,
    material: M,
}

impl<M> Plane<M> {
    pub fn from_triangle(point_a: Point3, point_b: Point3, point_c: Point3, material: M) -> Plane<M> {
        let ab = point_b - point_a;
        let bc = point_c - point_b;
        let normal = ab.cross(&bc);
        Plane {
            normal,
            normal_offset: normal.dot(&point_a.coords),
            material,
        }
    }
}

impl<M: Material + Sync> Object for Plane<M> {
    fn get_alpha(&self, ray: &Ray) -> Option<f64> {
        let orthogonality = self.normal.dot(&ray.direction);
        if orthogonality == 0.0 {
            return None;
        }
        let alpha = (self.normal_offset - self.normal.dot(&ray.start.coords)) / orthogonality;
        if alpha > 0.0 { Some(alpha) } else { None }
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