use crate::utils::{exif_utils::ExifFile, images::is_supported_img};
use log::{info, warn};
use pm_common::gallery_cache::{Orientation, Ratio};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use tauri::Window;

use super::windows_galleries::{WindowGallery, WindowsGalleriesState};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PictureCache {
    pub path: String,

    pub uuid_generated: bool,
    pub date: Option<String>,
    pub location_lat: Option<f64>,
    pub location_long: Option<f64>,
    pub location_alt: Option<f64>,
    pub camera: Option<String>,
    pub orientation: Orientation,
    pub focal_length: Option<f64>,
    pub exposure_time: Option<Ratio>,
    pub iso_speed: Option<i32>,
    pub f_number: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PathsCache {
    pub dir_name: String,
    pub children: Vec<PathsCache>,
    pub pictures: Vec<String>,
}

#[tauri::command]
pub fn update_gallery_cache(window: Window, galleries_state: tauri::State<WindowsGalleriesState>) -> (HashMap<String, PictureCache>, PathsCache) {
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
pub fn get_gallery_datas_cache(window: Window, galleries_state: tauri::State<WindowsGalleriesState>) -> HashMap<String, PictureCache> {
    let galleries = galleries_state.get_galleries();
    let gallery = WindowGallery::get(&galleries, &window);

    gallery.gallery.datas_cache.clone()
}
#[tauri::command]
pub fn get_gallery_paths_cache(window: Window, galleries_state: tauri::State<WindowsGalleriesState>) -> PathsCache {
    let galleries = galleries_state.get_galleries();
    let gallery = WindowGallery::get(&galleries, &window);

    gallery.gallery.paths_cache.clone()
}

pub fn read_dir_recursive(path: PathBuf, datas_cache: &mut HashMap<String, PictureCache>, gallery_path: &Path) -> PathsCache {
    let mut paths_cache = PathsCache::default();

    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            paths_cache.children.push(read_dir_recursive(path, datas_cache, gallery_path));
        } else if is_supported_img(path.clone()) {
            let stripped_path = path
                .strip_prefix(gallery_path)
                .expect("Can't strip image path prefix")
                .to_str()
                .expect("Non UTF-8 path")
                .to_string();

            if let Some(exif_file) = ExifFile::new(path.clone()) {
                datas_cache.insert(exif_file.uid.clone(), exif_file.to_picture_cache(stripped_path));
                paths_cache.pictures.push(exif_file.uid.clone());
            } else {
                warn!("File {:?} does not support EXIF of XMP data.", path);
            }
        }
    }
    paths_cache
}
