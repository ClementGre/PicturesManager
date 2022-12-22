use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PictureCache {
    pub path: String,
    pub location: String,
    pub date: String
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PathsCache {
    pub dir_name: String,
    pub children: Vec<PathsCache>,
    pub pictures: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationCache {
    pub name: String,
    pub children: Option<Vec<PathsCache>>,
    pub pictures: Option<Vec<String>>
}