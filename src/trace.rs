use image::ImageBuffer;
use image::Rgba;
use object::Object;
use prelude::*;
use ray::Ray;
use rayon::prelude::*;

pub struct Pixel {
    location: Point2,
    color: Color,
}

pub struct PixelField {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Pixel>,
}

impl PixelField {
    pub fn create(width: u32, height: u32, initial_color: Color) -> PixelField {
        let mut pixels = Vec::with_capacity(width as usize * height as usize);
        for y in 0..height {
            let y = 2.0 * (y as f64 + 0.5) / height as f64 - 1.0;
            for x in 0..width {
                let x = 2.0 * (x as f64 + 0.5) / width as f64 - 1.0;
                pixels.push(Pixel {
                    location: Point2::new(x, -y),
                    color: initial_color,
                });
            }
        }
        PixelField {
            width,
            height,
            pixels,
        }
    }

    pub fn render_scene(&mut self, scene: &[&Object]) {
        let start = Point3::new(0.0, 0.0, 1.0);

        self.pixels.par_iter_mut().for_each(|pixel| {
            let ray = Ray {
                start,
                direction: Vector3::new(pixel.location.x, pixel.location.y, -1.0),
            };

            pixel.color = ray.trace(&scene, 10);
        });
    }

    pub fn create_image_buffer(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut as_integers = Vec::with_capacity(3 as usize * self.width as usize * self.height as usize);
        for pixel in &self.pixels {
            as_integers.push((pixel.color.x.min(1.0) * 255.0) as u8);
            as_integers.push((pixel.color.y.min(1.0) * 255.0) as u8);
            as_integers.push((pixel.color.z.min(1.0) * 255.0) as u8);
            as_integers.push(255);
        }

        ImageBuffer::from_vec(self.width, self.height, as_integers).unwrap()
    }
}

pub mod file_output {}

pub mod piston_output {}
