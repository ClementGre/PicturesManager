use fast_image_resize as fr;
use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};
use log::info;
use std::io::{BufWriter, Cursor};
use std::num::NonZeroU32;
use std::{ffi::OsStr, path::PathBuf};

pub fn gen_thumbnail(path: &str, target_height: u32) -> Vec<u8> {
    let start = std::time::Instant::now();

    let img = ImageReader::open(path).unwrap().decode().unwrap();


    info!("Loading DynamicImage: {:?}", start.elapsed());
    let start = std::time::Instant::now();

    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();
    let src_image = fr::Image::from_vec_u8(width, height, img.to_rgba8().into_raw(), fr::PixelType::U8x4).unwrap();

    
    info!("Getting Image buffer: {:?}", start.elapsed());
    let start = std::time::Instant::now();

    // Create container for data of destination image
    let dst_width = NonZeroU32::new(target_height * img.width() / img.height()).unwrap();
    let dst_height = NonZeroU32::new(target_height).unwrap();
    let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

    // Get mutable view of destination image data
    let mut dst_view = dst_image.view_mut();

    // Resize source image
    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Box));
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();


    info!("Resizing: {:?}", start.elapsed());
    let start = std::time::Instant::now();


    // Write destination image as PNG-file
    let mut result_buf = BufWriter::new(Vec::new());
    PngEncoder::new(&mut result_buf)
        .write_image(dst_image.buffer(), dst_width.get(), dst_height.get(), ColorType::Rgba8)
        .unwrap();


    info!("Encoding png: {:?}", start.elapsed());

    result_buf.into_inner().unwrap().to_vec()
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