use super::{
    gallery_cache::{PathsCache, PictureCache},
    gallery_clusters::{DatesClusters, LocationClusters},
    gallery_tags::TagGroup
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Gallery {
    pub settings: GallerySettings,
    pub tag_groups: HashMap<String, TagGroup>,
    pub datas_cache: HashMap<String, PictureCache>,
    pub paths_cache: PathsCache,
    pub dates_cache: Vec<String>,
    pub dates_clusters: Vec<DatesClusters>,
    pub location_clusters: Vec<LocationClusters>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GallerySettings {
    test: String,
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
