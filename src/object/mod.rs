use prelude::*;
use ray::Ray;

pub mod plane;
pub mod primitive;
pub mod sphere;
pub mod sun;

pub trait Object: Sync {
    fn get_alpha(&self, ray: &Ray) -> Option<f64>;

    fn get_color(&self, ray: &Ray, alpha: f64, scene: &[&Object], num_recursions: usize) -> Color;
}
