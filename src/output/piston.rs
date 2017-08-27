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
use trace::ViewFrustum;

pub fn render_in_window<F>(mut field: ViewFrustum, scale: f64, mut update: F)
where
    F: FnMut(&mut ViewFrustum) + Send + 'static,
{
    let initial_image = field.create_image_buffer();

    let mut window: PistonWindow = WindowSettings::new(
        "Raytracer",
        [
            (field.width as f64 * scale) as u32,
            (field.height as f64 * scale) as u32,
        ],
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let mut texture = Texture::from_image(&mut window.factory, &initial_image, &TextureSettings::new()).unwrap();

    let shared_image = Arc::new(Mutex::new(initial_image));

    thread::spawn({
        let shared_image = shared_image.clone();
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
                *shared_image.lock().unwrap() = new_buffer;
            }
        }
    });

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            texture
                .update(&mut graphics.encoder, &shared_image.lock().unwrap())
                .unwrap();
            piston_window::image(&texture, context.transform.zoom(scale), graphics);
        });
    }
}
