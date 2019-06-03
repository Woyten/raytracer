use crate::trace::Camera;
use std::path::Path;

pub fn save_to_file(field: &Camera, file_name: &str) {
    field
        .create_image_buffer(None)
        .save(Path::new(file_name))
        .unwrap();
}
