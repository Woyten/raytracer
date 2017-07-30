use object::Object;
use prelude::*;
use std::f64;

pub struct Ray {
    pub start: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn trace(&self, scene: &[&Object], num_recursions: usize) -> Color {
        match self.find_nearest(scene) {
            Some((alpha, object)) => object.get_color(self, alpha, scene, num_recursions),
            None => Color::new(0.0, 0.0, 0.0),
        }
    }

    fn find_nearest<'a>(&self, scene: &'a [&Object]) -> Option<(f64, &'a Object)> {
        let mut nearest = f64::INFINITY;
        let mut nearest_object = None;
        for object in scene {
            if let Some(alpha) = object.get_alpha(self) {
                if alpha < nearest && alpha > 1e-9 {
                    nearest = alpha;
                    nearest_object = Some(object);
                }
            }
        }
        if let Some(object) = nearest_object { Some((nearest, *object)) } else { None }
    }
}

pub fn reflect(incident_direction: &Vector3, normal: &Vector3) -> Vector3 {
    incident_direction - 2.0 * incident_direction.dot(&normal) * normal / normal.norm_squared()
}