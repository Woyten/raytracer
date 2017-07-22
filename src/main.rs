extern crate image;
extern crate nalgebra;

use std::path::Path;
use image::ImageBuffer;
use image::Rgb;
use prelude::*;
use sphere::Sphere;
use ray::Ray;

mod prelude;
mod ray;
mod sphere;

struct Pixel {
    location: Point2,
    color: Color,
}

fn main() {
    let mut field = create_pixel_field(400, 400, (0.5, 0.5, 0.5));

    let sphere = Sphere {
        middle: Point3::new(1.0, 1.0, -1.0),
        radius: 0.5,
        color: (0.0, 1.0, 0.0),
    };

    let start = Point3::new(0.0, 0.0, 1.0);
    for pixel in &mut field {
        let ray = Ray {
            start,
            direction: Vector3::new(pixel.location.x, pixel.location.y, -1.0),
        };

        if sphere.has_intersection(&ray) {
            pixel.color = sphere.color;
        }
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
        as_integers.push((pixel.color.0 * 255.0) as u8);
        as_integers.push((pixel.color.1 * 255.0) as u8);
        as_integers.push((pixel.color.2 * 255.0) as u8);
    }

    ImageBuffer::<Rgb<_>, _>::from_vec(width, height, as_integers)
        .unwrap()
        .save(Path::new(file_name))
        .unwrap();
}
