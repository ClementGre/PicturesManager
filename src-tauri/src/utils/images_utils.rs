use fast_image_resize as fr;
use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};
use log::info;
use std::fs::{create_dir_all, read, write};
use std::io::BufWriter;
use std::num::NonZeroU32;
use std::{ffi::OsStr, path::PathBuf};
use tauri::{AppHandle, Window};

use crate::gallery::windows_galleries::{WindowGallery, WindowsGalleriesState};
use crate::header::window;

pub async fn get_thumbnail(gallery_path: String, image_path: String, id: String, target_height: u32) -> Option<Vec<u8>> {
    let thumb_path = PathBuf::from(&gallery_path).join(".thumbnails").join(format!("{}.png", &id));
    if let Ok(data) = read(thumb_path.clone()) {
        return Some(data);
    }
    let start = std::time::Instant::now();

    let img_path: PathBuf = PathBuf::from(&gallery_path).join(&image_path);
    let img = ImageReader::open(img_path).ok()?.decode().ok()?;

    let width = NonZeroU32::new(img.width())?;
    let height = NonZeroU32::new(img.height())?;
    let src_image = fr::Image::from_vec_u8(width, height, img.to_rgba8().into_raw(), fr::PixelType::U8x4).ok()?;

    // Create container for data of destination image
    let dst_width = NonZeroU32::new(target_height * img.width() / img.height())?;
    let dst_height = NonZeroU32::new(target_height)?;
    let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

    // Get mutable view of destination image data
    let mut dst_view = dst_image.view_mut();

    // Resize source image
    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Box));
    resizer.resize(&src_image.view(), &mut dst_view).ok()?;

    // Write destination image as PNG-file
    let mut result_buf = BufWriter::new(Vec::new());
    PngEncoder::new(&mut result_buf)
        .write_image(dst_image.buffer(), dst_width.get(), dst_height.get(), ColorType::Rgba8)
        .ok()?;

    let inner = result_buf.into_inner().ok()?;
    create_dir_all(thumb_path.parent()?).expect("Unable to create gallery directory.");
    write(thumb_path, &inner).ok()?;

    info!("Generating thumbnail took {:?}", start.elapsed());
    Some(inner)
}

#[tauri::command]
pub async fn get_image_thumbnail(app: AppHandle, window: Window, galleries_state: tauri::State<'_, WindowsGalleriesState>, id: String) -> Result<Vec<u8>, ()> {

    let path;
    let gallery_path;
    {
        let galleries = galleries_state.get_galleries();
        let gallery = WindowGallery::get(&galleries, &window);
        path = gallery.gallery.datas_cache.get(&id).unwrap().path.clone();
        gallery_path = gallery.path.clone();
    }

    Ok(get_thumbnail(gallery_path, path, id, 280).await.unwrap())
}

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
