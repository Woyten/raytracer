use object::Object;
use prelude::*;

pub mod diffuse;
pub mod transmissive;

pub trait Material {
    fn get_color(&self, direction: Vector3, reflection_point: Point3, normal: &Vector3, scene: &[&Object], num_recursions: usize) -> Color;
}
