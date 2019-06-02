use crate::trace::ViewFrustum;
use std::path::Path;

pub fn save_to_file(field: &ViewFrustum, file_name: &str) {
    field
        .create_image_buffer()
        .save(Path::new(file_name))
        .unwrap();
}
