use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use tauri::{State, Window, Wry};

use pm_common::gallery::{GalleryData, GallerySettings};

use crate::gallery::windows_galleries::{WindowGallery, WindowsGalleriesState};

use super::{
    gallery_cache::{PathsCache, PictureCache},
    gallery_clusters::{DatesClusters, LocationClusters},
    gallery_tags::TagGroup,
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Gallery {
    pub settings: GallerySettings,
    pub data: GalleryData,                     // Mainly frontend UI data
    pub tag_groups: HashMap<String, TagGroup>, // Tags groups by id

    pub datas_cache: HashMap<String, PictureCache>, // Pictures datas in function of their EXIF uid
    pub paths_cache: PathsCache,                    // Pictures EXIF uid, with directory structure (recursive structure)
    pub dates_cache: Vec<String>,                   // Pictures uid ordered by date

    pub dates_clusters: Vec<DatesClusters>,
    pub location_clusters: Vec<LocationClusters>,
}

impl Gallery {
    pub fn load(path: &String) -> Gallery {
        let file = PathBuf::from(path).join("pictures_manager.json");

        if file.exists() {
            let file = File::open(&file).expect("Unable to open gallery file");
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).expect("Unable to parse gallery file")
        } else {
            Gallery::default()
        }
    }
    pub fn save(&self, path: &String) {
        let dir = PathBuf::from(path);
        let file = dir.join("pictures_manager.json");

        create_dir_all(&dir).expect("Unable to create gallery directory.");

        let file = File::create(&file).expect("Unable to create gallery file");
        let writter = BufWriter::new(file);
        serde_json::to_writer_pretty(writter, &self).unwrap();
    }
}

#[tauri::command]
pub fn get_gallery_data(galleries: State<WindowsGalleriesState>, window: Window<Wry>) -> GalleryData {
    WindowGallery::get(&galleries.get_galleries(), &window).gallery.data.clone()
}
#[tauri::command]
pub fn set_gallery_data(galleries: State<WindowsGalleriesState>, window: Window<Wry>, data: GalleryData) {
    let mut galleries = galleries.get_galleries();
    WindowGallery::get_mut(&mut galleries, &window).gallery.data = data;
}

#[tauri::command]
pub fn get_gallery_settings(galleries: State<WindowsGalleriesState>, window: Window<Wry>) -> GallerySettings {
    WindowGallery::get(&galleries.get_galleries(), &window).gallery.settings.clone()
}
#[tauri::command]
pub fn set_gallery_settings(galleries: State<WindowsGalleriesState>, window: Window<Wry>, settings: GallerySettings) {
    let mut galleries = galleries.get_galleries();
    WindowGallery::get_mut(&mut galleries, &window).gallery.settings = settings;
}
