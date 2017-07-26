extern crate image;
extern crate nalgebra;

use image::ImageBuffer;
use image::Rgb;
use material::SimpleMaterial;
use object::Object;
use object::plane::Plane;
use object::primitive::Primitive;
use object::sphere::Sphere;
use object::sun::Sun;
use prelude::*;
use ray::Ray;
use std::path::Path;

mod material;
mod object;
mod prelude;
mod ray;

struct Pixel {
    location: Point2,
    color: Color,
}

fn main() {
    let mut field = create_pixel_field(400, 400, Color::new(0.0, 0.0, 0.0));

    let light_side = Vector3::new(1.0, 0.6, 1.0);
    let sphere1 = Sphere {
        middle: Point3::new(-0.5, -0.5, -1.0),
        radius: 0.5,
        material: SimpleMaterial {
            light_side,
            color_fn: |_| Color::new(0.1, 0.5, 0.1),
            reflectivity: 0.4,
        },
    };
    let sphere2 = Sphere {
        middle: Point3::new(0.5, -0.5, -1.0),
        radius: 0.5,
        material: SimpleMaterial {
            light_side: Vector3::new(1.0, 0.6, 1.0),
            color_fn: |_| Color::new(0.6, 0.2, 0.2),
            reflectivity: 0.4,
        },
    };
    let sphere3 = Sphere {
        middle: Point3::new(-0.5, 0.5, -1.0),
        radius: 0.5,
        material: SimpleMaterial {
            light_side,
            color_fn: |_| Color::new(0.2, 0.2, 0.9),
            reflectivity: 0.4,
        },
    };
    let primitive = Primitive::new(
        Point3::new(-1.0, 1.0, -0.5),
        Point3::new(1.0, 1.0, -0.5),
        Point3::new(0.0, 1.0, -1.0),
        SimpleMaterial {
            light_side,
            color_fn: |_| Color::new(0.2, 0.2, 0.2),
            reflectivity: 0.2,
        },
    );
    let checkerboard = Plane::from_triangle(
        Point3::new(-1.0, -1.0, -0.5),
        Point3::new(1.0, -1.0, -0.5),
        Point3::new(0.0, -1.0, -1.0),
        SimpleMaterial {
            light_side,
            color_fn: |point| 0.5 * Color::new(1.0, 1.0, 1.0) * if (point.x.abs() + 0.25).fract() < 0.5 { 1.0 } else { 0.0 },
            reflectivity: 0.3,
        },
    );
    let light = Sun {
        direction: light_side,
        color1: Color::new(0.0, 0.0, 0.0),
        color2: Color::new(1.0, 1.0, 0.7),
        threshold1: 0.96,
        threshold2: 0.99,
    };
    let scene = [
        &sphere1 as &Object,
        &sphere2 as &Object,
        &sphere3 as &Object,
        &primitive as &Object,
        &checkerboard as &Object,
        &light as &Object,
    ];

    let start = Point3::new(0.0, 0.0, 1.0);
    for pixel in &mut field {
        let ray = Ray {
            start,
            direction: Vector3::new(pixel.location.x, pixel.location.y, -1.0),
        };

        pixel.color = ray.trace(&scene, 10);
    }

    save_field(&field, 400, 400, "out.png");
}

fn create_pixel_field(width: u32, height: u32, initial_color: Color) -> Vec<Pixel> {
    let mut field = Vec::with_capacity(width as usize * height as usize);
    for y in 0..height {
        let y = 2.0 * (y as f64 + 0.5) / height as f64 - 1.0;
        for x in 0..width {
            let x = 2.0 * (x as f64 + 0.5) / width as f64 - 1.0;
            field.push(Pixel {
                location: Point2::new(x, -y),
                color: initial_color,
            });
        }
    }
    field
}

fn save_field(field: &Vec<Pixel>, width: u32, height: u32, file_name: &str) {
    let mut as_integers = Vec::with_capacity(3 as usize * width as usize * height as usize);
    for pixel in field {
        as_integers.push((pixel.color.x.min(1.0) * 255.0) as u8);
        as_integers.push((pixel.color.y.min(1.0) * 255.0) as u8);
        as_integers.push((pixel.color.z.min(1.0) * 255.0) as u8);
    }

    ImageBuffer::<Rgb<_>, _>::from_vec(width, height, as_integers)
        .unwrap()
        .save(Path::new(file_name))
        .unwrap();
}
