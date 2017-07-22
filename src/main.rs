extern crate image;

use std::path::Path;
use image::ImageBuffer;
use image::Rgb;

type Location = (f64, f64);

type Value = (f64, f64, f64);

struct Pixel {
    location: Location,
    value: Value,
}

fn main() {
    let mut field = create_pixel_field(400, 400);

    let radius = 0.5;

    for pixel in &mut field {
        if pixel.location.0 * pixel.location.0 + pixel.location.1 * pixel.location.1 <
            radius * radius
        {
            pixel.value.0 = 1.0;
        }
    }

    save_field(&field, 400, 400, "out.png");
}

fn create_pixel_field(width: u32, height: u32) -> Vec<Pixel> {
    let mut field = Vec::with_capacity(width as usize * height as usize);
    for y in 0..height {
        let y = 2.0 * (y as f64 + 0.5) / height as f64 - 1.0;
        for x in 0..width {
            let x = 2.0 * (x as f64 + 0.5) / width as f64 - 1.0;
            field.push(Pixel {
                location: (x, y),
                value: (0.0, 0.0, 0.0),
            });
        }
    }
    field
}

fn save_field(field: &Vec<Pixel>, width: u32, height: u32, file_name: &str) {
    let mut as_integers = Vec::with_capacity(3 as usize * width as usize * height as usize);
    for pixel in field {
        as_integers.push((pixel.value.0 * 255.0) as u8);
        as_integers.push((pixel.value.1 * 255.0) as u8);
        as_integers.push((pixel.value.2 * 255.0) as u8);
    }

    ImageBuffer::<Rgb<_>, _>::from_vec(width, height, as_integers)
        .unwrap()
        .save(Path::new(file_name))
        .unwrap();
}
