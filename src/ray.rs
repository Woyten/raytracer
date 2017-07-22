use f64;
use prelude::*;
use sphere::Sphere;

pub struct Ray {
    pub start: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn trace<'a>(&self, scene: &'a Vec<Sphere>) -> (f64, Option<&'a Sphere>) {
        let mut nearest = f64::MAX;
        let mut nearest_sphere = None;
        for sphere in scene {
            if let Some(alpha) = sphere.get_intersection_distance(&self) {
                if alpha < nearest {
                    nearest = alpha;
                    nearest_sphere = Some(sphere);
                }
            }
        }
        return (nearest, nearest_sphere);
    }
}