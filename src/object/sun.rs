use object::Object;
use prelude::*;
use ray::Ray;
use std::f64;

pub struct Sun {
    pub direction: Vector3,
    pub color1: Color,
    pub color2: Color,
    pub threshold1: f64,
    pub threshold2: f64,
}

impl Object for Sun {
    fn get_alpha(&self, _: &Ray, _: f64) -> Option<f64> {
        Some(f64::MAX)
    }

    fn get_color(&self, ray: &Ray, _: f64, _: &[&Object], _: usize) -> Color {
        let dot_product = ray.direction.normalize().dot(&self.direction.normalize());
        if dot_product < self.threshold1 {
            self.color1
        } else if dot_product > self.threshold2 {
            self.color2
        } else {
            ((self.threshold2 - dot_product) * self.color1 + (dot_product - self.threshold1) * self.color2) / (self.threshold2 - self.threshold1)
        }
    }
}
