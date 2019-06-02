use crate::material::diffuse::Diffuse;
use crate::material::transmissive::Transmissive;
use crate::object::plane::Plane;
use crate::object::primitive::Primitive;
use crate::object::sphere::Sphere;
use crate::object::sun::Sun;
use crate::object::Object;
use crate::output::piston;
use crate::prelude::*;
use crate::trace::ViewFrustum;
use nalgebra::geometry::Rotation3;
use raytracer::*;
use std::f64;

fn main() {
    let mut angle = 0.0;
    let initial = ViewFrustum::create(800, 800, Color::new(0.0, 0.0, 0.0));
    piston::render_in_window(initial, 1.0, move |field| {
        angle += 0.01;
        render_scene(field, angle);
    });
}

fn render_scene(field: &mut ViewFrustum, angle: f64) {
    let light_side = Vector3::new(1.0, -0.6, 1.0);

    let green_sphere = Sphere {
        middle: Point3::new(-0.25, 0.25, 0.0),
        radius: 0.25,
        material: Diffuse {
            light_side,
            color_fn: |_| Color::new(0.1, 0.5, 0.1),
            reflectivity: 0.4,
        },
    };

    let red_sphere = Sphere {
        middle: Point3::new(0.25, 0.25, 0.0),
        radius: 0.25,
        material: Diffuse {
            light_side,
            color_fn: |_| Color::new(0.6, 0.2, 0.2),
            reflectivity: 0.4,
        },
    };

    let floor = Plane::from_triangle(
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(1.0, 0.0, -1.0),
        Point3::new(0.0, 1.0, -1.0),
        Diffuse {
            light_side,
            color_fn: |point| {
                0.5 * Color::new(1.0, 1.0, 1.0)
                    * if (point.x.abs() + 0.25).fract() < 0.5 {
                        1.0
                    } else {
                        0.0
                    }
            },
            reflectivity: 0.3,
        },
    );

    let left_tip = Point3::new(-0.25, -0.25, -0.25);
    let right_tip = Point3::new(0.25, -0.25, -0.25);
    let front_tip = Point3::new(0.0, -0.5, -0.0);
    let top_tip = Point3::new(0.0, -0.25, 0.25);

    let left_face = Primitive::new(
        left_tip,
        front_tip,
        top_tip,
        Transmissive {
            color: Color::new(0.8, 1.0, 1.0),
            reflectivity: 0.2,
            refraction: 1.4,
        },
    );
    let right_face = Primitive::new(
        front_tip,
        right_tip,
        top_tip,
        Transmissive {
            color: Color::new(0.8, 1.0, 1.0),
            reflectivity: 0.2,
            refraction: 1.4,
        },
    );
    let back_face = Primitive::new(
        right_tip,
        left_tip,
        top_tip,
        Transmissive {
            color: Color::new(0.8, 1.0, 1.0),
            reflectivity: 0.2,
            refraction: 1.4,
        },
    );
    let bottom_face = Primitive::new(
        left_tip,
        right_tip,
        front_tip,
        Transmissive {
            color: Color::new(0.8, 1.0, 1.0),
            reflectivity: 0.2,
            refraction: 1.4,
        },
    );

    let light = Sun {
        direction: light_side,
        color1: Color::new(0.0, 0.0, 0.0),
        color2: Color::new(1.0, 1.0, 0.7),
        threshold1: 0.96,
        threshold2: 0.99,
    };

    let scene: [&dyn Object; 8] = [
        &green_sphere,
        &red_sphere,
        &floor,
        &left_face,
        &right_face,
        &back_face,
        &bottom_face,
        &light,
    ];

    let transform = Rotation3::from_euler_angles(80.0 * f64::consts::PI / 180.0, 0.0, angle);
    field.render_scene(&transform.matrix(), &scene);
}
