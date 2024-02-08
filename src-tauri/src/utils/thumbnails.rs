use std::fs::{create_dir_all, read, write};
use std::io::BufWriter;
use std::num::NonZeroU32;
use std::{ffi::OsStr, path::PathBuf};

use fast_image_resize as fr;
use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};
use log::{info, warn};
use tauri::{Window, Wry};

use pm_common::gallery_cache::Orientation;

use crate::gallery::windows_galleries::{WindowGallery, WindowsGalleriesState};
use crate::utils::files_utils::path_from_unix_path_string;

// First called function to determine image dimension
// Dimensions are in the right orientation
#[tauri::command]
pub fn get_image_dimensions(window: Window<Wry>, galleries_state: tauri::State<'_, WindowsGalleriesState>, id: String) -> Option<(u32, u32)> {
    let galleries = galleries_state.get_galleries();
    let gallery = WindowGallery::get(&galleries, &window);
    let cache = gallery.gallery.datas_cache.get(&id)?;
    let (w, h) = cache.dimensions;

    Some(match cache.orientation {
        Orientation::Rotate90 | Orientation::Rotate270 | Orientation::Rotate90HorizontalFlip | Orientation::Rotate90VerticalFlip => (h, w),
        _ => (w, h),
    })
}

// Second called function to make sure thumbnail exists
#[tauri::command]
pub async fn gen_image_thumbnail(window: Window<Wry>, galleries_state: tauri::State<'_, WindowsGalleriesState>, id: String) -> Result<bool, ()> {
    let path;
    let gallery_path;
    let orientation;
    {
        let galleries = galleries_state.get_galleries();
        let gallery = WindowGallery::get(&galleries, &window);
        orientation = gallery.gallery.datas_cache.get(&id).unwrap().orientation;
        path = path_from_unix_path_string(gallery.gallery.datas_cache.get(&id).unwrap().path.clone());
        gallery_path = gallery.path.clone();
    }
    Ok(gen_thumbnail(gallery_path, path, id, orientation, 280).await.is_some())
}
async fn gen_thumbnail(gallery_path: String, image_path: String, id: String, orientation: Orientation, target_height: u32) -> Option<()> {
    // Check if thumbnail already exists
    let thumb_path = PathBuf::from(&gallery_path).join(".thumbnails").join(format!("{}.png", &id));
    if thumb_path.exists() {
        return Some(());
    }
    let start = std::time::Instant::now();

    let img_path: PathBuf = PathBuf::from(&gallery_path).join(&image_path);
    let img = ImageReader::open(img_path.clone());
    if let Err(e) = img {
        warn!("Unable to open image: {:?}, error: {}", img_path, e);
        return None;
    }
    let img = img.ok()?.decode();
    if let Err(e) = img {
        warn!("Unable to decode image: {:?}, error: {}", img_path, e);
        return None;
    }
    let img = img.ok()?;

    // Rotate image if needed
    let img = match orientation {
        Orientation::Rotate90 => img.rotate90(),
        Orientation::Rotate180 => img.rotate180(),
        Orientation::Rotate270 => img.rotate270(),
        Orientation::Rotate90HorizontalFlip => img.rotate90().fliph(),
        Orientation::Rotate90VerticalFlip => img.rotate90().flipv(),
        Orientation::HorizontalFlip => img.fliph(),
        Orientation::VerticalFlip => img.flipv(),
        _ => img,
    };

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

    create_dir_all(thumb_path.parent()?).expect("Unable to create gallery directory.");
    write(thumb_path, result_buf.into_inner().ok()?).ok()?;

    info!("Generating thumbnail took {:?}", start.elapsed());

    Some(())
}

// Third called function to get thumbnail data through custom protocol
pub fn get_existing_thumbnail(gallery_path: &str, id: &str) -> Option<Vec<u8>> {
    let thumb_path = PathBuf::from(&gallery_path).join(".thumbnails").join(format!("{}.png", &id));
    if let Ok(data) = read(thumb_path.clone()) {
        return Some(data);
    }
    None
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
