use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct PathsCache {
    pub dir_name: String,
    pub children: Vec<PathsCache>,
    pub pictures: Vec<String>,
}


#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Orientation {
    #[default]
    Unspecified,
    Normal,
    HorizontalFlip,
    Rotate180,
    VerticalFlip,
    Rotate90HorizontalFlip,
    Rotate90,
    Rotate90VerticalFlip,
    Rotate270,
}