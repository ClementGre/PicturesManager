use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Store)]
#[serde(default)]
pub struct GalleryData {
    pub current_left_tab: u16,
    pub files_tab_selected_dir: Vec<String>,
    pub zoom_grid: f64,
    pub zoom_carousel: f64,
}
impl Default for GalleryData {
    fn default() -> Self {
        Self {
            current_left_tab: 0,
            files_tab_selected_dir: vec![],
            zoom_grid: 1.0,
            zoom_carousel: 1.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[serde(default)]
pub struct GallerySettings {
    pub test: String,
}
