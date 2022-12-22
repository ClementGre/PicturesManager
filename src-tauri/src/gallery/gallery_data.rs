use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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