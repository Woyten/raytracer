use image::ImageBuffer;
use image::Rgb;
use object::Object;
use prelude::*;
use ray::Ray;
use std::path::Path;

struct Pixel {
    location: Point2,
    color: Color,
}

pub fn render_scene_to_file(scene: &[&Object], file_name: &str, width: u32, height: u32) {
    let mut field = create_pixel_field(width, height, Color::new(0.0, 0.0, 0.0));

    let start = Point3::new(0.0, 0.0, 1.0);
    for pixel in &mut field {
        let ray = Ray {
            start,
            direction: Vector3::new(pixel.location.x, pixel.location.y, -1.0),
        };

        pixel.color = ray.trace(&scene, 10);
    }

    save_field(&field, width, height, file_name);
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
