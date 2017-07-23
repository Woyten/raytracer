use f64;
use prelude::*;
use sphere::Sphere;

pub struct Ray {
    pub start: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn trace<'a>(&self, scene: &'a Vec<Sphere>, num_recursions: usize) -> Color {
        match self.find_nearest(&scene) {
            Some((alpha, sphere)) => sphere.get_color(self, alpha, scene, num_recursions),
            None => Color::new(0.0, 0.0, 0.0),
        }
    }

    fn find_nearest<'a>(&self, scene: &'a Vec<Sphere>) -> Option<(f64, &'a Sphere)> {
        let mut nearest = f64::MAX;
        let mut nearest_sphere = None;
        for sphere in scene {
            if let Some(alpha) = sphere.get_alpha(&self) {
                if alpha < nearest {
                    nearest = alpha;
                    nearest_sphere = Some(sphere);
                }
            }
        }
        if let Some(sphere) = nearest_sphere { Some((nearest, sphere)) } else { None }
    }
}