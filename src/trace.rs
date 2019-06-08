use crate::object::Object;
use crate::prelude::*;
use crate::ray::Ray;
use image::ImageBuffer;
use image::Rgba;
use log::debug;
use rayon::prelude::*;

pub struct Pixel {
    ray: Ray,
    color: Color,
}

pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Pixel>,
}

impl Camera {
    pub fn create(width: u32, height: u32, initial_color: Color) -> Camera {
        let start = Point3::new(0.0, 0.0, 1.0);

        let mut pixels = Vec::with_capacity(width as usize * height as usize);
        for y in 0..height {
            let y = 2.0 * (f64::from(y) + 0.5) / f64::from(height) - 1.0;
            for x in 0..width {
                let x = 2.0 * (f64::from(x) + 0.5) / f64::from(width) - 1.0;

                let location = Point2::new(x, -y);

                let ray = Ray {
                    start,
                    direction: Vector3::new(location.x, location.y, -1.0),
                };

                pixels.push(Pixel {
                    ray,
                    color: initial_color,
                });
            }
        }
        Camera {
            width,
            height,
            pixels,
        }
    }

    pub fn render_scene(&mut self, transform: &Matrix3, scene: &[&dyn Object]) {
        self.pixels.par_iter_mut().for_each(|pixel| {
            let transformed_ray = Ray {
                start: transform * pixel.ray.start,
                direction: transform * pixel.ray.direction,
            };
            pixel.color = transformed_ray.trace(&scene, 10);
        });
    }

    pub fn create_image_buffer<B: Into<Option<Vec<u8>>>>(
        &self,
        buffer_to_reuse: B,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let time = std::time::Instant::now();

        let mut buffer = buffer_to_reuse.into().unwrap_or_else(Vec::new);
        buffer.resize(4 * self.width as usize * self.height as usize, 0);

        buffer
            .par_chunks_mut(4)
            .zip_eq(self.pixels.par_iter())
            .for_each(|(chunk, pixel)| {
                chunk.copy_from_slice(&[
                    (pixel.color.x.min(1.0) * 255.0) as u8,
                    (pixel.color.y.min(1.0) * 255.0) as u8,
                    (pixel.color.z.min(1.0) * 255.0) as u8,
                    255,
                ]);
            });

        debug!("Elaspsed: {:?}", time.elapsed());

        ImageBuffer::from_vec(self.width, self.height, buffer).unwrap()
    }
}
