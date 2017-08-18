use piston_window;
use piston_window::PistonWindow;
use piston_window::Texture;
use piston_window::TextureSettings;
use piston_window::Transformed;
use piston_window::WindowSettings;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;
use trace::PixelField;

pub fn render_in_window<F>(mut field: PixelField, scale: f64, mut update: F)
where
    F: FnMut(&mut PixelField) + Send + 'static,
{
    let mut window: PistonWindow = WindowSettings::new(
        "Raytracer",
        [
            (field.width as f64 * scale) as u32,
            (field.height as f64 * scale) as u32,
        ],
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let image_buffer = Arc::new(Mutex::new(field.create_image_buffer()));

    thread::spawn({
        let image_buffer = image_buffer.clone();
        move || {
            let start = Instant::now();
            let mut frames = 0;
            loop {
                update(&mut field);

                frames += 1;
                let duration = start.elapsed();
                let duration_in_secs = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
                println!("FPS: {:.1}", frames as f64 / duration_in_secs);

                let new_buffer = field.create_image_buffer();
                *image_buffer.lock().unwrap() = new_buffer;
            }
        }
    });

    let mut texture = Texture::from_image(&mut window.factory, &image_buffer.lock().unwrap(), &TextureSettings::new()).unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            texture
                .update(&mut graphics.encoder, &image_buffer.lock().unwrap())
                .unwrap();
            piston_window::image(&texture, context.transform.zoom(scale), graphics);
        });
    }
}
