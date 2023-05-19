use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
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


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Ratio {
    pub numer: i32,
    pub denom: i32,
}
impl Ratio {
    pub fn from_num_rational(ratio: num_rational::Ratio<i32>) -> Self {
        Self {
            numer: *ratio.numer(),
            denom: *ratio.denom(),
        }
    }
}