use std::path::Path;
use trace::PixelField;

pub fn save_to_file(field: &PixelField, file_name: &str) {
    field
        .create_image_buffer()
        .save(Path::new(file_name))
        .unwrap();
}
