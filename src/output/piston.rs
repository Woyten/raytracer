use crate::trace::Camera;
use log::info;
use piston_window;
use piston_window::PistonWindow;
use piston_window::Texture;
use piston_window::TextureSettings;
use piston_window::Transformed;
use piston_window::WindowSettings;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

pub fn render_in_window<F>(mut field: Camera, scale: f64, mut update: F)
where
    F: FnMut(&mut Camera) + Send + 'static,
{
    let initial_image = field.create_image_buffer(None);

    let mut window: PistonWindow = WindowSettings::new(
        "Raytracer",
        [
            (f64::from(field.width) * scale) as u32,
            (f64::from(field.height) * scale) as u32,
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let shared_image = Arc::new(Mutex::new(initial_image));

    thread::spawn({
        let shared_image = shared_image.clone();
        let mut buffer = Vec::new();
        move || {
            let start = Instant::now();
            let mut frames = 0;
            loop {
                update(&mut field);

                frames += 1;
                let duration = start.elapsed();
                let duration_in_secs =
                    duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) * 1e-9;
                info!("FPS: {:.1}", f64::from(frames) / duration_in_secs);

                let new_buffer = field.create_image_buffer(buffer);
                let old_buffer =
                    std::mem::replace(shared_image.lock().unwrap().deref_mut(), new_buffer);
                buffer = old_buffer.into_raw();
            }
        }
    });

    let mut texture_context = window.create_texture_context();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            // TODO: Update texture
            let texture = Texture::from_image(
                &mut texture_context,
                &shared_image.lock().unwrap(),
                &TextureSettings::new(),
            )
            .unwrap();

            piston_window::image(&texture, context.transform.zoom(scale), graphics);
        });
    }
}
