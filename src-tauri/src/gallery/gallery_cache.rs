use crate::utils::exif_utils::{ExifFile, Orientation, Ratio};
use log::warn;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use tauri::Window;

use super::{windows_galleries::{WindowsGalleriesState, WindowGallery}, gallery_data::Gallery};

#[derive(Debug, Serialize, Deserialize, Default)]
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PathsCache {
    pub dir_name: String,
    pub children: Vec<PathsCache>,
    pub pictures: Vec<String>,
}

#[tauri::command]
pub fn update_gallery_cache(window: Window, galleries_state: tauri::State<WindowsGalleriesState>) {
    let mut galleries = galleries_state.galleries.lock().unwrap();

    let mut gallery = galleries
        .iter()
        .find(|gallery| gallery.window_label == window.label())
        .expect("Unable to find gallery in galleries list.");

    let gallery_path = Path::new(&gallery.path);
    let mut datas_cache = HashMap::new();
    let paths_cache = read_dir_recursive(PathBuf::from(&gallery.path), &mut datas_cache, gallery_path);

    // TODO: Update gallery data
}

pub fn read_dir_recursive(path: PathBuf, datas_cache: &mut HashMap<String, PictureCache>, gallery_path: &Path) -> PathsCache {
    let mut paths_cache = PathsCache::default();

    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            paths_cache.children.push(read_dir_recursive(path, datas_cache, gallery_path));
        } else {
            paths_cache.pictures.push(String::from(path.to_str().unwrap()));

            if let Some(exif_file) = ExifFile::new(path.clone()) {
                let stripped_path = path
                    .strip_prefix(gallery_path)
                    .expect("Can't strip image path prefix")
                    .to_str()
                    .unwrap()
                    .to_string();
                datas_cache.insert(exif_file.uid.clone(), exif_file.to_picture_cache(stripped_path));
            } else {
                warn!("File {:?} does not support EXIF of XMP data.", path);
            }
        }
    }

    paths_cache
}
