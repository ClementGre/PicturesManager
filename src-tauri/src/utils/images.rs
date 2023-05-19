use std::{ffi::OsStr, path::PathBuf};

const SUPPORTED_EXTENSIONS: [&str; 6] = ["png", "jpg", "jpeg", "gif", "bmp", "webp"];

pub fn is_supported_img_ext(ext: &OsStr) -> bool {
    SUPPORTED_EXTENSIONS.iter().any(|e| *e == ext.to_str().unwrap_or_default().to_lowercase())
}

pub fn is_supported_img(path: PathBuf) -> bool {
    if let Some(extension) = path.extension() {
        is_supported_img_ext(extension)
    } else {
        false
    }
}
