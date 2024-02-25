use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use log::{info, warn};
use serde::{Deserialize, Serialize};
use tauri::{Window, Wry};

use pm_common::gallery_cache::Orientation;

use crate::utils::exif_utils::ExifFile;
use crate::utils::files_utils::{path_from_unix_path_string, path_to_unix_path_string};
use crate::utils::thumbnails::is_supported_img;

use super::windows_galleries::{WindowGallery, WindowsGalleriesState};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PictureCache {
    pub path: String, // In unix style, call get_path to get a valid path.
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

impl PictureCache {
    pub fn get_path(&self) -> String {
        path_from_unix_path_string(self.path.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct PathsCache {
    pub dir_name: String,
    pub children: Vec<PathsCache>, // Subdirectories, sorted by name
    pub pictures: Vec<String>,     // EXIF uid, sorted by date
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

pub fn update_gallery_cache(
    window: &Window<Wry>,
    galleries_state: &tauri::State<WindowsGalleriesState>,
) -> (HashMap<String, PictureCache>, PathsCache) {
    let mut galleries = galleries_state.get_galleries();
    let gallery = WindowGallery::get_mut(&mut galleries, &window);

    let start = std::time::Instant::now();

    let gallery_path = Path::new(&gallery.path);
    let mut datas_cache = HashMap::new();
    let mut dates_cache = Vec::new();
    let paths_cache = read_dir_recursive(PathBuf::from(&gallery.path), &mut datas_cache, &mut dates_cache, gallery_path);

    dates_cache.sort_by(|(_, a), (_, b)| a.cmp(b));
    let dates_cache: Vec<String> = dates_cache.iter().map(|(uid, _)| uid.clone()).collect();

    let elapsed = start.elapsed();
    info!("Gallery cache updated {} pictures in {}ms", datas_cache.len(), elapsed.as_millis());

    gallery.gallery.datas_cache = datas_cache.clone();
    gallery.gallery.paths_cache = paths_cache.clone();
    gallery.gallery.dates_cache = dates_cache.clone();
    gallery.gallery.save(&gallery.path);

    (datas_cache, paths_cache)
}

pub fn read_dir_recursive(
    path: PathBuf,
    datas_cache: &mut HashMap<String, PictureCache>,
    dates_cache: &mut Vec<(String, Option<String>)>,
    gallery_path: &Path,
) -> PathsCache {
    let mut paths_cache = PathsCache {
        dir_name: path.file_name().unwrap().to_str().unwrap().to_string(),
        ..Default::default()
    };
    let paths = fs::read_dir(path).unwrap();
    let mut pictures = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            if !path.file_name().unwrap().to_str().unwrap().starts_with('.') {
                paths_cache
                    .children
                    .push(read_dir_recursive(path, datas_cache, dates_cache, gallery_path));
            }
        } else if is_supported_img(path.clone()) {
            let stripped_path = path.strip_prefix(gallery_path).expect("Can't strip image path prefix");

            if let Some(mut exif_file) = ExifFile::new(path.clone()) {
                // TODO: check that the file is the same as an old one (edit date, name, etc.) because it might have been edited. In this case, the uid should be regenerated.
                if datas_cache.contains_key(&exif_file.uid) {
                    info!("Regenerating uid for file {:?} because this uid already exists.", path);
                    exif_file.regen_uid();
                }
                dates_cache.push((exif_file.uid.clone(), exif_file.get_date()));
                datas_cache.insert(exif_file.uid.clone(), exif_file.to_picture_cache(path_to_unix_path_string(stripped_path)));
                pictures.push((exif_file.uid.clone(), exif_file.get_date()));
            } else {
                warn!("File {:?} does not support EXIF of XMP data.", path);
            }
        }
    }
    pictures.sort_by(|(_, a), (_, b)| a.cmp(b));
    paths_cache.pictures = pictures.iter().map(|(uid, _)| uid.clone()).collect();
    paths_cache.children.sort_by(|a, b| a.dir_name.cmp(&b.dir_name));
    paths_cache
}
