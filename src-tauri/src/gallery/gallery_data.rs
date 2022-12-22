use std::{collections::HashMap, fs::{File, create_dir_all}, io::{BufReader, BufWriter}, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use super::gallery_cache::{PictureCache, PathsCache, LocationCache};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Gallery {
    settings: GallerySettings,
    tag_groups: HashMap<String, TagGroup>,
    cache: HashMap<String, PictureCache>,
    cache_dates: Vec<String>,
    cache_paths: PathsCache,
    cache_location: Vec<LocationCache>
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GallerySettings {
    test: String
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TagGroup {
    pub name: String,
    pub multiple: bool,
    pub tags: HashMap<String, Tag>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tag {
    pub name: String,
    pub color: String,
    pub pictures: Vec<String>,
}

impl Gallery {
    pub fn load(path: String) -> Gallery {
        let file = PathBuf::from(path).join("pictures_manager.json");

        if file.exists() {
            let file = File::open(&file).expect("Unable to open gallery file");
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).expect("Unable to parse gallery file")
        }else {
            Gallery::default()
        }
    }
    pub fn save(&self, path: String) {
        let dir = PathBuf::from(path);
        let file = dir.join("pictures_manager.json");

        create_dir_all(&dir).expect("Unable to create gallery directory.");

        let file = File::create(&file).expect("Unable to create gallery file");
        let writter = BufWriter::new(file);
        serde_json::to_writer(writter, &self).unwrap();
    }
}