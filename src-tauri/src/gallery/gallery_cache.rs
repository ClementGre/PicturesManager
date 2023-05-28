use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use log::{info, warn};
use serde::{Deserialize, Serialize};
use tauri::{Window, Wry};

use pm_common::gallery_cache::Orientation;

use crate::utils::{exif_utils::ExifFile, thumbnails::is_supported_img};

use super::windows_galleries::{WindowGallery, WindowsGalleriesState};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PictureCache {
    pub path: String,
    pub uuid_generated: bool,
    pub date: Option<String>,
    pub location: Option<(f64, f64, f64)>,
    pub orientation: Orientation,
    pub dimensions: (u32, u32),
    pub camera: Option<String>,
    pub focal_length: Option<f64>,
    pub exposure_time: Option<(u32, u32)>,
    pub iso_speed: Option<i32>,
    pub f_number: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PathsCache {
    pub dir_name: String,
    pub children: Vec<PathsCache>,
    pub pictures: Vec<String>,
}

pub fn update_gallery_cache(
    window: &Window<Wry>,
    galleries_state: tauri::State<WindowsGalleriesState>,
) -> (HashMap<String, PictureCache>, PathsCache) {
    let mut galleries = galleries_state.get_galleries();
    let gallery = WindowGallery::get_mut(&mut galleries, &window);

    let start = std::time::Instant::now();

    let gallery_path = Path::new(&gallery.path);
    let mut datas_cache = HashMap::new();
    let paths_cache = read_dir_recursive(PathBuf::from(&gallery.path), &mut datas_cache, gallery_path);

    let elapsed = start.elapsed();
    info!("Gallery cache updated {} pictures in {}ms", datas_cache.len(), elapsed.as_millis());

    gallery.gallery.datas_cache = datas_cache.clone();
    gallery.gallery.paths_cache = paths_cache.clone();
    gallery.gallery.save(&gallery.path);

    (datas_cache, paths_cache)
}
#[tauri::command]
pub fn get_gallery_datas_cache(window: Window<Wry>, galleries_state: tauri::State<WindowsGalleriesState>) -> HashMap<String, PictureCache> {
    let galleries = galleries_state.get_galleries();
    let gallery = WindowGallery::get(&galleries, &window);

    gallery.gallery.datas_cache.clone()
}
#[tauri::command]
pub fn get_gallery_paths_cache(window: Window<Wry>, galleries_state: tauri::State<WindowsGalleriesState>) -> PathsCache {
    let galleries = galleries_state.get_galleries();
    let gallery = WindowGallery::get(&galleries, &window);

    gallery.gallery.paths_cache.clone()
}

pub fn read_dir_recursive(path: PathBuf, datas_cache: &mut HashMap<String, PictureCache>, gallery_path: &Path) -> PathsCache {
    let mut paths_cache = PathsCache {
        dir_name: path.file_name().unwrap().to_str().unwrap().to_string(),
        ..Default::default()
    };
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            if !path.file_name().unwrap().to_str().unwrap().starts_with('.') {
                paths_cache.children.push(read_dir_recursive(path, datas_cache, gallery_path));
            }
        } else if is_supported_img(path.clone()) {
            let stripped_path = path
                .strip_prefix(gallery_path)
                .expect("Can't strip image path prefix")
                .to_str()
                .expect("Non UTF-8 path")
                .to_string();

            if let Some(mut exif_file) = ExifFile::new(path.clone()) {
                if datas_cache.contains_key(&exif_file.uid) {
                    // TODO: compare locations with old cache and regenerate uid for the file that changed location (or was renamed)
                    exif_file.regen_uid();
                }
                datas_cache.insert(exif_file.uid.clone(), exif_file.to_picture_cache(stripped_path));
                paths_cache.pictures.push(exif_file.uid.clone());
            } else {
                warn!("File {:?} does not support EXIF of XMP data.", path);
            }
        }
    }
    paths_cache
}
